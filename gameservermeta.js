import Web3 from 'web3';
 
import util from 'util';
import { createServer } from "http";
import { Server } from "socket.io";
//import { emit } from 'process';
import { ApiPromise, WsProvider } from '@polkadot/api';
//import { web3Accounts, web3Enable, web3FromAddress } from '@polkadot/extension-dapp';
import { cryptoWaitReady, mnemonicGenerate, mnemonicValidate, mnemonicToMiniSecret, sr25519PairFromSeed } from '@polkadot/util-crypto';
import { Keyring } from '@polkadot/keyring';
import { ContractPromise } from '@polkadot/api-contract';

/// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ ///
///
///       CONNECTED USERS + PLAYER DATA              
///
/// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ ///

const clients = [];

/// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ ///
///
///       SERVER PORT
///
/// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ ///
const httpServer = createServer();
const io = new Server(httpServer);

httpServer.listen(9945, function(){
  console.log('listening on *:9945');
});



/// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ ///
///
///       WALLET & SOCKET INFO
///
/// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ ///

const mainAddr = "5ECfUBcQMMvfd9WVjbnaY3m96AQqyut13sHPnYSirKVfCrEY"; //HELIOS
let pubKey = "";
const provider = new WsProvider('ws://127.0.0.1:9944');
const abi = {
  "source": {
    "hash": "0x9b6334c6df2ac1a860439102f74b1a2db9ba75d83883f59a761a0f833a490246",
    "language": "ink! 3.0.0-rc9",
    "compiler": "rustc 1.61.0-nightly"
  },
  "contract": {
    "name": "vrmetax1",
    "version": "0.1.0",
    "authors": [
      "[your_name] <[your_email]>"
    ]
  },
  "V3": {
    "spec": {
      "constructors": [
        {
          "args": [],
          "docs": [],
          "label": "new",
          "payable": true,
          "selector": "0x9bae9d5e"
        }
      ],
      "docs": [],
      "events": [],
      "messages": [
        {
          "args": [],
          "docs": [],
          "label": "buy_ammo",
          "mutates": true,
          "payable": true,
          "returnType": null,
          "selector": "0x2b867bd0"
        },
        {
          "args": [],
          "docs": [],
          "label": "buy_missiles",
          "mutates": true,
          "payable": true,
          "returnType": null,
          "selector": "0x67056a47"
        },
        {
          "args": [],
          "docs": [],
          "label": "buy_gun_rights",
          "mutates": true,
          "payable": true,
          "returnType": null,
          "selector": "0x07dc73ab"
        },
        {
          "args": [],
          "docs": [],
          "label": "buy_nft_skin",
          "mutates": true,
          "payable": true,
          "returnType": null,
          "selector": "0xbbe18393"
        },
        {
          "args": [
            {
              "label": "amount_bullets",
              "type": {
                "displayName": [
                  "Balance"
                ],
                "type": 7
              }
            }
          ],
          "docs": [],
          "label": "shoot_ammo",
          "mutates": true,
          "payable": false,
          "returnType": null,
          "selector": "0x0b035174"
        },
        {
          "args": [],
          "docs": [
            " Return ammo"
          ],
          "label": "get_ammo",
          "mutates": false,
          "payable": false,
          "returnType": {
            "displayName": [
              "Balance"
            ],
            "type": 7
          },
          "selector": "0x880133dc"
        },
        {
          "args": [],
          "docs": [
            " Return ammo"
          ],
          "label": "get_missiles",
          "mutates": false,
          "payable": false,
          "returnType": {
            "displayName": [
              "Balance"
            ],
            "type": 7
          },
          "selector": "0x49bbed78"
        },
        {
          "args": [],
          "docs": [],
          "label": "get_gun_rights",
          "mutates": false,
          "payable": false,
          "returnType": {
            "displayName": [
              "bool"
            ],
            "type": 9
          },
          "selector": "0xf66010dc"
        },
        {
          "args": [],
          "docs": [],
          "label": "get_owns_nft_skin",
          "mutates": false,
          "payable": false,
          "returnType": {
            "displayName": [
              "bool"
            ],
            "type": 9
          },
          "selector": "0x8a9bbfea"
        }
      ]
    },
    "storage": {
      "struct": {
        "fields": [
          {
            "layout": {
              "cell": {
                "key": "0x0000000000000000000000000000000000000000000000000000000000000000",
                "ty": 0
              }
            },
            "name": "points"
          },
          {
            "layout": {
              "cell": {
                "key": "0x0100000000000000000000000000000000000000000000000000000000000000",
                "ty": 1
              }
            },
            "name": "master_address"
          },
          {
            "layout": {
              "cell": {
                "key": "0x0200000000000000000000000000000000000000000000000000000000000000",
                "ty": 6
              }
            },
            "name": "ammo"
          },
          {
            "layout": {
              "cell": {
                "key": "0x0300000000000000000000000000000000000000000000000000000000000000",
                "ty": 6
              }
            },
            "name": "missiles"
          },
          {
            "layout": {
              "cell": {
                "key": "0x0400000000000000000000000000000000000000000000000000000000000000",
                "ty": 8
              }
            },
            "name": "gun_rights"
          },
          {
            "layout": {
              "cell": {
                "key": "0x0500000000000000000000000000000000000000000000000000000000000000",
                "ty": 8
              }
            },
            "name": "nft_skins"
          }
        ]
      }
    },
    "types": [
      {
        "id": 0,
        "type": {
          "def": {
            "composite": {
              "fields": [
                {
                  "name": "offset_key",
                  "type": 5,
                  "typeName": "Key"
                }
              ]
            }
          },
          "params": [
            {
              "name": "K",
              "type": 1
            },
            {
              "name": "V",
              "type": 4
            }
          ],
          "path": [
            "ink_storage",
            "lazy",
            "mapping",
            "Mapping"
          ]
        }
      },
      {
        "id": 1,
        "type": {
          "def": {
            "composite": {
              "fields": [
                {
                  "type": 2,
                  "typeName": "[u8; 32]"
                }
              ]
            }
          },
          "path": [
            "ink_env",
            "types",
            "AccountId"
          ]
        }
      },
      {
        "id": 2,
        "type": {
          "def": {
            "array": {
              "len": 32,
              "type": 3
            }
          }
        }
      },
      {
        "id": 3,
        "type": {
          "def": {
            "primitive": "u8"
          }
        }
      },
      {
        "id": 4,
        "type": {
          "def": {
            "primitive": "u32"
          }
        }
      },
      {
        "id": 5,
        "type": {
          "def": {
            "composite": {
              "fields": [
                {
                  "type": 2,
                  "typeName": "[u8; 32]"
                }
              ]
            }
          },
          "path": [
            "ink_primitives",
            "Key"
          ]
        }
      },
      {
        "id": 6,
        "type": {
          "def": {
            "composite": {
              "fields": [
                {
                  "name": "offset_key",
                  "type": 5,
                  "typeName": "Key"
                }
              ]
            }
          },
          "params": [
            {
              "name": "K",
              "type": 1
            },
            {
              "name": "V",
              "type": 7
            }
          ],
          "path": [
            "ink_storage",
            "lazy",
            "mapping",
            "Mapping"
          ]
        }
      },
      {
        "id": 7,
        "type": {
          "def": {
            "primitive": "u128"
          }
        }
      },
      {
        "id": 8,
        "type": {
          "def": {
            "composite": {
              "fields": [
                {
                  "name": "offset_key",
                  "type": 5,
                  "typeName": "Key"
                }
              ]
            }
          },
          "params": [
            {
              "name": "K",
              "type": 1
            },
            {
              "name": "V",
              "type": 9
            }
          ],
          "path": [
            "ink_storage",
            "lazy",
            "mapping",
            "Mapping"
          ]
        }
      },
      {
        "id": 9,
        "type": {
          "def": {
            "primitive": "bool"
          }
        }
      }
    ]
  }
}




const buyAmmo = async(value) => {
  const contractADDR = '5CN58Z2tiabSoTULM2KYBznKKnmKmzgum4ge5L7zSaoFAJCS';
  const api = await ApiPromise.create({ provider });
  const keyring = new Keyring({ type: 'sr25519' });
  const helios = keyring.addFromUri('doll pizza govern cart thunder gentle pulse century coin suggest carbon shock');

  // We will use these values for the execution
  const valueFixed = value * (10 ** 9); // only useful on isPayable messages

  const contract = new ContractPromise(api, abi, contractADDR);
  //const injector = await web3FromAddress(mainAddr);
   await contract.tx
  .buyAmmo({ valueFixed, gasLimit: -1 })
  .signAndSend(helios, (result) => {
    if (result.status.isInBlock) {
      console.log('in a block');
    } else if (result.status.isFinalized) {
      console.log('finalized');
    }
  });
  io.emit('ammo bought', 'Success');
}

const buyMissiles = async(value) => {
  const contractADDR = '5CN58Z2tiabSoTULM2KYBznKKnmKmzgum4ge5L7zSaoFAJCS';
  const api = await ApiPromise.create({ provider });
  const keyring = new Keyring({ type: 'sr25519' });
  const helios = keyring.addFromUri('doll pizza govern cart thunder gentle pulse century coin suggest carbon shock');

  // We will use these values for the execution
  const valueFixed = value * (10 ** 9); // only useful on isPayable messages

  const contract = new ContractPromise(api, abi, contractADDR);
  //const injector = await web3FromAddress(mainAddr);
   await contract.tx
  .buyMissiles({ valueFixed, gasLimit: -1 })
  .signAndSend(helios, (result) => {
    if (result.status.isInBlock) {
      console.log('in a block');
    } else if (result.status.isFinalized) {
      console.log('finalized');
    }
  });
  io.emit('missiles bought', 'Success');
}

async function main () {
  // Initialise the provider to connect to the local node
 
  const api = await ApiPromise.create({ provider });

  const now = await api.query.timestamp.now();
  const nowReadable = now.toNumber();
  console.log("BlockNumber:  "+nowReadable);

  io.emit('current num', "Block:  "+nowReadable);
}

async function getBalance() {
	const api = await ApiPromise.create({ provider });

	const balance  = await api.query.system.account(mainAddr);
	let readableBalance = balance.data.free.toNumber();
	console.log("Balance: "+readableBalance);
  io.emit('get balance',  (readableBalance / (10 **9))+ " VRMETA");
	
}

async function connectToGame() {
	 const api = await ApiPromise.create({ provider });
   // Constuct the keyring after the API (crypto has an async init)
   const keyring = new Keyring({ type: 'sr25519' });

   // Add Alice to our keyring with a hard-deived path (empty phrase, so uses dev)
   const helios = keyring.addFromUri('doll pizza govern cart thunder gentle pulse century coin suggest carbon shock');

  await api.tx.timestake.connect().signAndSend(helios, (result) => {
    if (result.status.isInBlock) {
      console.log('in a block');
    } else if (result.status.isFinalized) {
      console.log('finalized');
      
    }
  });

	io.emit('match started',  "Ok");	
}

async function disconnectFromGame() {
  const api = await ApiPromise.create({ provider });
  // Constuct the keyring after the API (crypto has an async init)
  const keyring = new Keyring({ type: 'sr25519' });

  // Add Alice to our keyring with a hard-deived path (empty phrase, so uses dev)
  const helios = keyring.addFromUri('doll pizza govern cart thunder gentle pulse century coin suggest carbon shock');

 await api.tx.timestake.disconnect().signAndSend(helios, (result) => {
   if (result.status.isInBlock) {
     console.log('in a block');
   } else if (result.status.isFinalized) {
     console.log('finalized');
     
   }
 });

 io.emit('match ended',  "Ok");	
}

async function createWallet() {

    await cryptoWaitReady();
    const mnemonic = mnemonicGenerate(12);
    const seed = mnemonic;

      // Validate the mnemic string that was generated
    const isValidMnemonic = mnemonicValidate(mnemonic);


// Create valid Substrate-compatible seed from mnemonic
    const seedUser = mnemonicToMiniSecret(mnemonic);
    const keyring = new Keyring({ type: 'sr25519', ss58Format: 2 });

// Generate new public/secret keypair for Alice from the supplied seed
     const { publicKey, secretKey } = sr25519PairFromSeed(seedUser);
    const publicKey1 = keyring.encodeAddress(publicKey);
    const secretKey1 = secretKey;

const pair = keyring.createFromUri(mnemonic);
console.log(mnemonic);
console.log(publicKey);



pubKey = publicKey1;

io.emit('wallet created', "New wallet address: "+ pubKey);


}


/// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ ///
///
///       CONNECTION + COMMUNICATION
///           FUNCTION LIBRARY
///          'OK' = READY TO USE
///
/// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ ///


io.on('connection', function(socket){

 clients.push(socket.id);

 var clientConnectedMsg = 'User connected ' + util.inspect(socket.id) + ', total: ' + clients.length;
 console.log(clientConnectedMsg);



/// OK
socket.on('get balance', () => {
	getBalance();
});

/// OK
socket.on('create wallet', () => {
	createWallet();
});

socket.on('shoot', () => {
  calculateAmmo();
});

socket.on('buy ammo', (value) => {
  buyAmmo(value);
});

socket.on('buy missiles', (value) => {
  buyMissiles(value);
})


socket.on('start match', () => {
  connectToGame();
});

socket.on('end match', () => {
	disconnectFromGame();
});

socket.on('buy nft', (value) => {
  buyNFT(value);
});

/// OK
socket.on('disconnect', async function(){
  clients.pop(socket.id);

  var clientDisconnectedMsg = 'User disconnected ' + util.inspect(socket.id) + ', total: ' + clients.length;
  console.log(clientDisconnectedMsg);
  
 })
});



/// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ ///
///
///       REPEATED FUNCTIONS IN MILLISECONDS
///
/// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ ///

setInterval(main, 2000);
setInterval(getBalance, 2000);

//setInterval(getData, 6000);