import detectEthereumProvider from "@metamask/detect-provider"
import { Strategy, ZkIdentity } from "@zk-kit/identity"
import { generateMerkleProof, Semaphore } from "@zk-kit/protocols"
import { Contract, providers, utils } from "ethers"
import Head from "next/head"
import React from "react"
import styles from "../styles/Home.module.css"

import { useForm } from "react-hook-form";
import { Button, TextField, Typography } from '@mui/material';
import { object, string, number } from 'yup';
import Greeter from "artifacts/contracts/Greeters.sol/Greeters.json"

export default function Home() {

    const [logs, setLogs] = React.useState("Connect your wallet and greet!")
    const [detectedGreeting, setDetectedGreeting] = React.useState();

    const [yupError, setYupError] = React.useState();

    const { register, handleSubmit, watch, formState: { errors } } = useForm();

    const formSchema = object({
      name: string().required(),
      age: number().required().positive().integer().max(199, "Sorry, but you must be at most ${max} years old to submit"),
      address: string().required().test(
        'is-eth-address',
        'Address must be a valid ethereum address',
        (value, context) => utils.isAddress(value),
      )
    });

    React.useEffect(() => {

      const provider = new providers.JsonRpcProvider("http://localhost:8545")
      const contract = new Contract("0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512", Greeter.abi, provider)

      contract.on("NewGreeting", (newGreeting) => {
        console.log("New Greeting detected !")
        const theGreeting = utils.parseBytes32String(newGreeting);
        console.log(theGreeting);
        setDetectedGreeting(theGreeting);
      })

      return () => {
        console.log('Removing listeners')
        contract.removeAllListeners();
      }

    }, [])

    const onSubmit = async (data) => {

      try{
        setYupError(null);
        await formSchema.validate(data)
        console.log(data);
      }catch(e){
        setYupError(e?.message)
      }

    }

    async function greet() {
        setLogs("Creating your Semaphore identity...")

        const provider = (await detectEthereumProvider()) as any

        await provider.request({ method: "eth_requestAccounts" })

        const ethersProvider = new providers.Web3Provider(provider)
        const signer = ethersProvider.getSigner()
        const message = await signer.signMessage("Sign this message to create your identity!")

        const identity = new ZkIdentity(Strategy.MESSAGE, message)
        const identityCommitment = identity.genIdentityCommitment()
        const identityCommitments = await (await fetch("./identityCommitments.json")).json()

        const merkleProof = generateMerkleProof(20, BigInt(0), identityCommitments, identityCommitment)

        setLogs("Creating your Semaphore proof...")

        const greeting = "Hello world"

        const witness = Semaphore.genWitness(
            identity.getTrapdoor(),
            identity.getNullifier(),
            merkleProof,
            merkleProof.root,
            greeting
        )

        const { proof, publicSignals } = await Semaphore.genProof(witness, "./semaphore.wasm", "./semaphore_final.zkey")
        const solidityProof = Semaphore.packToSolidityProof(proof)

        const response = await fetch("/api/greet", {
            method: "POST",
            body: JSON.stringify({
                greeting,
                nullifierHash: publicSignals.nullifierHash,
                solidityProof: solidityProof
            })
        })

        if (response.status === 500) {
            const errorMessage = await response.text()

            setLogs(errorMessage)
        } else {
            setLogs("Your anonymous greeting is onchain :)")
        }
    }

    return (
        <div className={styles.container}>
            <Head>
                <title>Greetings</title>
                <meta name="description" content="A simple Next.js/Hardhat privacy application with Semaphore." />
                <link rel="icon" href="/favicon.ico" />
            </Head>

            <main className={styles.main}>
                <h1 className={styles.title}>Greetings</h1>

                <p className={styles.description}>A simple Next.js/Hardhat privacy application with Semaphore.</p>

                <div className={styles.logs}>{logs}</div>

                <div onClick={() => greet()} className={styles.button}>
                    Greet
                </div>

                {
                  detectedGreeting &&
                  <div className={styles.alignText}>
                    <p></p>
                    <Typography>New Greeting Detected !</Typography>
                    <Typography>{detectedGreeting}</Typography>
                  </div>
                }

                {/* "handleSubmit" will validate your inputs before invoking "onSubmit" */}
                <form className={styles.form} onSubmit={handleSubmit(onSubmit)}>

                  {yupError &&
                    <div className={styles.alignText}>
                      <Typography>Yup validation error:</Typography>
                      <Typography>{yupError}</Typography>
                    </div>
                  }

                  {/* register your input into the hook by invoking the "register" function */}
                  <TextField
                    label="Name"
                    variant="filled"
                    color="warning"
                    // defaultValue="test"
                    {...register("name", { required: true, pattern: /^[A-Za-z]+$/i })}
                  />
                  {errors?.name?.type === 'required' && <span>Name is required</span>}
                  {errors?.name?.type === 'pattern' && <span>Invalid Name</span>}

                  {/* include validation with required or other standard HTML validation rules */}
                  <TextField
                    label="Age"
                    type="number"
                    variant="filled"
                    color="warning"
                    {...register("age", { required: true, min: 0, max: 200 })}
                  />
                  {/* errors will return when field validation fails  */}
                  {errors?.age?.type === 'required' && <span>Age is required</span>}
                  {(errors?.age?.type === 'min' || errors?.age?.type === 'max') && <span>Age Invalid</span>}

                  <TextField
                    label="Address"
                    variant="filled"
                    color="warning"
                    {...register("address", { required: true })}
                  />
                  {/* errors will return when field validation fails  */}
                  {errors?.address?.type === 'required' && <span>Address is required</span>}

                  <Button
                    color="warning"
                    type="submit"
                  >
                    Submit
                  </Button>

                </form>

            </main>
        </div>
    )
}
