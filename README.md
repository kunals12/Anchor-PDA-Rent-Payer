# Key Takeaways

## SystemAccount:
Represents a Solana account owned by the System Program. These accounts can be created, initialized, and funded using CPI calls to the System Program.

## PDA (rent_vault):
A Program Derived Address (PDA) is a special account owned by the program. It requires seeds and a bump to resolve deterministically.

## CPI (Cross-Program Invocation):
The program invokes the System Program to perform operations (transfer, create_account).

## Signers and Authorization:
The with_signer method ensures the PDA can authorize its lamport transfers.