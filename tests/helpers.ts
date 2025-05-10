import { Connection, PublicKey } from "@solana/web3.js";
import { getAssociatedTokenAddressSync, getAccount, TokenAccountNotFoundError, TokenInvalidAccountOwnerError, createAssociatedTokenAccountIdempotentInstruction } from "@solana/spl-token";

export const getOrCreateATAInstruction = async (
    connection: Connection,
    tokenMint: PublicKey,
    owner: PublicKey,
    payer: PublicKey = owner,
    allowOwnerOffCurve = true
): Promise<{ ataPubKey: PublicKey; ix: any }> => {
    const toAccount = getAssociatedTokenAddressSync(
        tokenMint,
        owner,
        allowOwnerOffCurve
    );

    try {
        await getAccount(connection, toAccount);
        return { ataPubKey: toAccount, ix: undefined };
    } catch (e) {
        if (
            e instanceof TokenAccountNotFoundError ||
            e instanceof TokenInvalidAccountOwnerError ||
            e.message.includes("Could not find") // Bankrun connection proxy error
        ) {
            const ix = createAssociatedTokenAccountIdempotentInstruction(
                payer,
                toAccount,
                owner,
                tokenMint
            );
            return { ataPubKey: toAccount, ix };
        }
        throw e;
    }
}; 