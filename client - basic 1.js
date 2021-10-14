// client.js is used to introduce the reader to generating clients from IDLs.
// It is not expected users directly test with this example. For a more
// ergonomic example, see `tests/basic-0.js` in this workspace.

//const anchor = require('./Programming/Rust2/anchorclone/anchor-1');

const anchor = require('@project-serum/anchor');
// Configure the local cluster.
anchor.setProvider(anchor.Provider.local());






async function main() {
  // #region main
  // Read the generated IDL.
  /* idl subcommand provides commands for interacting with interface definition files. 
  It's recommended to use these commands to store an IDL on chain, at a deterministic address, as a function of nothing but the the program's ID.
  This allow us to generate clients for a program using nothing but the program ID.*/
  const idl = JSON.parse(require('fs').readFileSync('./target/idl/basic_0.json', 'utf8'));

  // The program to execute.
const program = anchor.workspace.Basic1;

// The Account to create.
const myAccount = anchor.web3.Keypair.generate();

  // Address of the deployed program.
  const programId = new anchor.web3.PublicKey('BSx34g4SKLYBrVdGuaw7xJz1yY6dLRhz9r2ZwwQjM1WY');

  // Generate the program client from IDL.
  //a client is the initiator of a request over a network
  const program = new anchor.Program(idl, programId);

  // Execute the RPC.
  /*Remote Procedure Call (RPC) is a protocol that one program can use to request a service
   from a program located in another computer on a network without having to understand the network's details.*/
   
  await program.rpc.initialize();
  // #end region main

  // Create the new account and initialize it with the program.
await program.rpc.initialize(new anchor.BN(1234), {
  accounts: {
    myAccount: myAccount.publicKey,
    user: provider.wallet.publicKey,
    systemProgram: SystemProgram.programId,
  },
  signers: [myAccount],
});

}

console.log('Running client.');
main().then(() => console.log('Success'));
