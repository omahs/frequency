import "@frequency-chain/api-augment";
import assert from "assert";
import { createKeys, createAndFundKeypair, signPayloadSr25519, Sr25519Signature, generateAddKeyPayload, fundKeypair, devAccounts } from "../scaffolding/helpers";
import { KeyringPair } from "@polkadot/keyring/types";
import { AddKeyData, ExtrinsicHelper } from "../scaffolding/extrinsicHelpers";
import { u64 } from "@polkadot/types";
import { Codec } from "@polkadot/types/types";
import { EXISTENTIAL_DEPOSIT } from "../scaffolding/rootHooks";
import { firstValueFrom } from "rxjs";
import { BlockHash, CreatedBlock } from "@polkadot/types/interfaces";

describe.skip("MSA Initializer Load Tests", function () {
    let keys: KeyringPair;

    before(async function () {
        const keypair = createKeys();
        ExtrinsicHelper.transferFunds(devAccounts[0].keys, keypair, EXISTENTIAL_DEPOSIT).signAndSend(100);
        await ExtrinsicHelper.api.rpc.engine.createBlock(true, true);
        keys = keypair

    });

    describe("createMsa", function () {
        it("should successfully create 10 signatures within a block", async function () {
            let msaId = getMsaId(keys)

            for (let i = 0; i < 10; i++) {
                await addSigs(msaId, keys);
            }
            await ExtrinsicHelper.api.rpc.engine.createBlock(true, true);
        });

        it("should successfully create 100 signatures within a block", async function () {
            let msaId = getMsaId(keys)

            for (let i = 0; i < 100; i++) {
                await addSigs(msaId, keys);
            }
            await ExtrinsicHelper.api.rpc.engine.createBlock(true, true);
        });

        it("should successfully create 1000 signatures within a block", async function () {
            let msaId = getMsaId(keys)

            for (let i = 0; i < 1000; i++) {
                await addSigs(msaId, keys);
            }
            await ExtrinsicHelper.api.rpc.engine.createBlock(true, true);
        });

        it("should successfully create 10000 signatures within a block", async function () {
            let msaId = getMsaId(keys)

            for (let i = 0; i < 10000; i++) {
                await addSigs(msaId, keys);
            }
            await ExtrinsicHelper.api.rpc.engine.createBlock(true, true);
        });
    });
})

async function addSigs(msaId, keys) {
    let newkeys = await createAndFundKeypair();
    let defaultPayload: AddKeyData = {};
    defaultPayload.msaId = msaId;
    defaultPayload.newPublicKey = newkeys.publicKey;
    let payload = await generateAddKeyPayload(defaultPayload);
    let addKeyData = ExtrinsicHelper.api.registry.createType("PalletMsaAddKeyData", payload);

    let ownerSig = signPayloadSr25519(keys, addKeyData);
    let newSig = signPayloadSr25519(newkeys, addKeyData);
    const addPublicKeyOp = ExtrinsicHelper.addPublicKeyToMsa(keys, ownerSig, newSig, payload);

    const [publicKeyEvents] = await addPublicKeyOp.fundAndSend();

    assert.notEqual(publicKeyEvents, undefined, 'should have added public key');
}

async function getMsaId(keys) {
    const f = ExtrinsicHelper.createMsa(keys);
    // await ExtrinsicHelper.api.rpc.engine.createBlock(true, true);
    await f.fundOperation();
    // await ExtrinsicHelper.api.rpc.engine.createBlock(true, true);
    const [msaCreatedEvent, chainEvents] = await f.signAndSend(200);
    await ExtrinsicHelper.api.rpc.engine.createBlock(true, true);

    // if (msaCreatedEvent && ExtrinsicHelper.api.events.msa.MsaCreated.is(msaCreatedEvent)) {
    return msaCreatedEvent?.data["msaId"];
    // }
}

async function getBlockEvents(blockHash: BlockHash) {
    const signedBlock = await ExtrinsicHelper.api.rpc.chain.getBlock(blockHash);
    const blockEvents = await ExtrinsicHelper.api.query.system.events()
    return firstValueFrom(blockEvents);
}