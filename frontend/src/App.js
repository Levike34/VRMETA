import React, { useState, createRef, useEffect } from 'react'
import './App.css';
import {
  Container,
  Dimmer,
  Loader,
  Grid,
  Sticky,
  Message,
} from 'semantic-ui-react'
import 'semantic-ui-css/semantic.min.css'
import Generate from './Generate.js'
import Swal from 'sweetalert2'

import { web3Accounts, web3Enable, web3FromAddress } from '@polkadot/extension-dapp';
import { SubstrateContextProvider, useSubstrateState } from './substrate-lib'
import { DeveloperConsole } from './substrate-lib/components'
import { ContractPromise } from '@polkadot/api-contract'
import { Keyring } from '@polkadot/api'
import AccountSelector from './AccountSelector'
import Balances from './Balances'
import BlockNumber from './BlockNumber'
import Events from './Events'
import Interactor from './Interactor'
import Metadata from './Metadata'
import NodeInfo from './NodeInfo'
import TemplateModule from './TemplateModule'
import Transfer from './Transfer'
import Upgrade from './Upgrade'

const DECIMALS = 1000000000;

function Main() {
  const { api, apiState, apiError, keyringState } = useSubstrateState()
  const [page, setPage] = useState(true);
  const [time, setTime] = useState();
  const [balance, setBalance] = useState();
  const [hourlyReward, setHourlyReward] = useState();
  const [myTimePlayed, setMyTimePlayed] = useState();
  const [alice, setAlice] = useState();
  const [connected, setConnected] = useState("");
  const [mainAccount, setMainAccount] = useState();
  const [rewardToExpect, setRewardToExpect] = useState();

  useEffect(() => {
    updateInfo();

    const interval = setInterval(()=>{
      updateInfo()
     },6000)
       
       
     return()=>clearInterval(interval)
  }, [])
 
  const abi = {
    "source": {
      "hash": "0xcd52becae811b64af1f9bfb29ac275f7935a0b934121e8db4204ca572f786a04",
      "language": "ink! 3.0.0-rc9",
      "compiler": "rustc 1.61.0-nightly"
    },
    "contract": {
      "name": "timestake",
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
        "events": [
          {
            "args": [
              {
                "docs": [],
                "indexed": true,
                "label": "caller",
                "type": {
                  "displayName": [
                    "Option"
                  ],
                  "type": 13
                }
              },
              {
                "docs": [],
                "indexed": true,
                "label": "timestamp",
                "type": {
                  "displayName": [
                    "Option"
                  ],
                  "type": 14
                }
              }
            ],
            "docs": [],
            "label": "Connected"
          },
          {
            "args": [
              {
                "docs": [],
                "indexed": true,
                "label": "caller",
                "type": {
                  "displayName": [
                    "Option"
                  ],
                  "type": 13
                }
              },
              {
                "docs": [],
                "indexed": true,
                "label": "timestamp",
                "type": {
                  "displayName": [
                    "Option"
                  ],
                  "type": 14
                }
              },
              {
                "docs": [],
                "indexed": true,
                "label": "reward_to_pay",
                "type": {
                  "displayName": [
                    "Option"
                  ],
                  "type": 14
                }
              }
            ],
            "docs": [],
            "label": "Disconnected"
          }
        ],
        "messages": [
          {
            "args": [],
            "docs": [],
            "label": "connect",
            "mutates": true,
            "payable": false,
            "returnType": null,
            "selector": "0x6dea651a"
          },
          {
            "args": [],
            "docs": [],
            "label": "disconnect",
            "mutates": true,
            "payable": true,
            "returnType": null,
            "selector": "0x6e33a829"
          },
          {
            "args": [],
            "docs": [
              " Simply returns the current value of our reward per hour."
            ],
            "label": "get_reward_hourly",
            "mutates": false,
            "payable": false,
            "returnType": {
              "displayName": [
                "u64"
              ],
              "type": 7
            },
            "selector": "0x159173b6"
          },
          {
            "args": [
              {
                "label": "tokens_per_hour",
                "type": {
                  "displayName": [
                    "u64"
                  ],
                  "type": 7
                }
              }
            ],
            "docs": [],
            "label": "set_reward_hourly",
            "mutates": true,
            "payable": false,
            "returnType": {
              "displayName": [
                "Result"
              ],
              "type": 10
            },
            "selector": "0x5b1426d0"
          },
          {
            "args": [],
            "docs": [],
            "label": "get_total_balance",
            "mutates": false,
            "payable": false,
            "returnType": {
              "displayName": [
                "u128"
              ],
              "type": 9
            },
            "selector": "0xe9c89f50"
          },
          {
            "args": [],
            "docs": [],
            "label": "get_start_time",
            "mutates": false,
            "payable": false,
            "returnType": {
              "displayName": [
                "u64"
              ],
              "type": 7
            },
            "selector": "0xcedd67fa"
          },
          {
            "args": [],
            "docs": [
              " Calculate time played in seconds"
            ],
            "label": "get_time_played",
            "mutates": false,
            "payable": false,
            "returnType": {
              "displayName": [
                "u64"
              ],
              "type": 7
            },
            "selector": "0xdea24990"
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
              "name": "is_connected"
            },
            {
              "layout": {
                "cell": {
                  "key": "0x0100000000000000000000000000000000000000000000000000000000000000",
                  "ty": 6
                }
              },
              "name": "start_time"
            },
            {
              "layout": {
                "cell": {
                  "key": "0x0200000000000000000000000000000000000000000000000000000000000000",
                  "ty": 6
                }
              },
              "name": "end_time"
            },
            {
              "layout": {
                "cell": {
                  "key": "0x0300000000000000000000000000000000000000000000000000000000000000",
                  "ty": 8
                }
              },
              "name": "reward"
            },
            {
              "layout": {
                "cell": {
                  "key": "0x0400000000000000000000000000000000000000000000000000000000000000",
                  "ty": 7
                }
              },
              "name": "reward_rate_per_hour"
            },
            {
              "layout": {
                "cell": {
                  "key": "0x0500000000000000000000000000000000000000000000000000000000000000",
                  "ty": 1
                }
              },
              "name": "owner"
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
              "primitive": "bool"
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
              "primitive": "u64"
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
              "primitive": "u128"
            }
          }
        },
        {
          "id": 10,
          "type": {
            "def": {
              "variant": {
                "variants": [
                  {
                    "fields": [
                      {
                        "type": 11
                      }
                    ],
                    "index": 0,
                    "name": "Ok"
                  },
                  {
                    "fields": [
                      {
                        "type": 12
                      }
                    ],
                    "index": 1,
                    "name": "Err"
                  }
                ]
              }
            },
            "params": [
              {
                "name": "T",
                "type": 11
              },
              {
                "name": "E",
                "type": 12
              }
            ],
            "path": [
              "Result"
            ]
          }
        },
        {
          "id": 11,
          "type": {
            "def": {
              "tuple": []
            }
          }
        },
        {
          "id": 12,
          "type": {
            "def": {
              "variant": {
                "variants": [
                  {
                    "index": 0,
                    "name": "NonOwner"
                  }
                ]
              }
            },
            "path": [
              "timestake",
              "timestake",
              "Error"
            ]
          }
        },
        {
          "id": 13,
          "type": {
            "def": {
              "variant": {
                "variants": [
                  {
                    "index": 0,
                    "name": "None"
                  },
                  {
                    "fields": [
                      {
                        "type": 1
                      }
                    ],
                    "index": 1,
                    "name": "Some"
                  }
                ]
              }
            },
            "params": [
              {
                "name": "T",
                "type": 1
              }
            ],
            "path": [
              "Option"
            ]
          }
        },
        {
          "id": 14,
          "type": {
            "def": {
              "variant": {
                "variants": [
                  {
                    "index": 0,
                    "name": "None"
                  },
                  {
                    "fields": [
                      {
                        "type": 7
                      }
                    ],
                    "index": 1,
                    "name": "Some"
                  }
                ]
              }
            },
            "params": [
              {
                "name": "T",
                "type": 7
              }
            ],
            "path": [
              "Option"
            ]
          }
        }
      ]
    }
  }
  function numberWithCommas(x) {
    return x.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",");
  }
  const loader = text => (
    <Dimmer active>
      <Loader size="small">{text}</Loader>
    </Dimmer>
  )

  const updateInfo = async() => {
// we are now informed that the user has at least one extension and that we
// will be able to show and use accounts
    await web3Enable('my cool dapp');
    const allAccounts = await web3Accounts();
    const mainAddr = allAccounts[0].address;

    // Retrieve last block timestamp, account nonce & balances
    const contractADDR = '5F9MB58PdzDNQkvMwAQipsEJfK1kQS1EozLd2bXPFRAHhrPF';

    // Retrieve the last timestamp
    const now = await api.query.timestamp.now();

    // Retrieve the account balance & nonce via the system module
    const balance  = await api.query.system.account(mainAddr);
    let readableBalance = balance.data.free.toNumber();
    

    const contract = new ContractPromise(api, abi, contractADDR);

    const timePlayed = await (await contract.query.getTimePlayed(mainAddr, {value: 0, gasLimit: -1})).output.toNumber();
    let readableTimePlayed = (Math.round((timePlayed / 60000) * 100) / 100).toFixed(2);
   
    const callValue = await (await contract.query.getRewardHourly(mainAddr, {value: 0, gasLimit: -1})).output.toJSON();
   

    setHourlyReward(callValue);
    setTime(numberWithCommas(now.toJSON()));
    setBalance(readableBalance / DECIMALS);
    setAlice(alice);
    setMainAccount(mainAddr);
    setMyTimePlayed(readableTimePlayed);
    setRewardToExpect(Math.round((timePlayed / 3600000) * 100) / 100).toFixed(2); //3,600,000 seconds in an hour in blocktime.
    if (timePlayed > 0) {
      setConnected('Yes');
    } else {
      setConnected('No');
    }

  }

  const connect = async() => {
    const contractADDR = '5F9MB58PdzDNQkvMwAQipsEJfK1kQS1EozLd2bXPFRAHhrPF';

    // We will use these values for the execution
    const value = 0; // only useful on isPayable messages

    const contract = new ContractPromise(api, abi, contractADDR);

    const injector = await web3FromAddress(mainAccount);
  
     await contract.tx
    .connect({ value, gasLimit: -1 })
    .signAndSend(mainAccount, {signer: injector.signer}, (result) => {
      if (result.status.isInBlock) {
        console.log('in a block');
      } else if (result.status.isFinalized) {
        console.log('finalized');
      }
    });
    setConnected('Yes');
  }

  const disconnect = async() => {
    const contractADDR = '5F9MB58PdzDNQkvMwAQipsEJfK1kQS1EozLd2bXPFRAHhrPF';
   // const contract = new ContractPromise(api, abi, contractADDR);

    // We will use these values for the execution
    const value = 0; // only useful on isPayable messages

    const contract = new ContractPromise(api, abi, contractADDR);

    const injector = await web3FromAddress(mainAccount);
  
     await contract.tx
    .disconnect({ value, gasLimit: -1 })
    .signAndSend(mainAccount, {signer: injector.signer}, (result) => {
      if (result.status.isInBlock) {
        console.log('in a block');
      } else if (result.status.isFinalized) {
        console.log('finalized');
        Swal.fire({
          title: 'Disconnected from the Metaverse:',
          text: 'You have mined '+rewardToExpect+" VRMETA.  Come again soon.",
          color: 'black',
        })
        
      }
    });
    setConnected('No');
    
  }

  const message = errObj => (
    <Grid centered columns={2} padded>
      <Grid.Column>
        <Message
          negative
          compact
          floating
          header="Error Connecting to Substrate"
          content={`Connection to websocket '${errObj.target.url}' failed.`}
        />
      </Grid.Column>
    </Grid>
  )

  const handlePage = () => {
    if(page == false) {
      setPage(true);
      console.log(api.consts.balances.existentialDeposit.toNumber());
      updateInfo();
    } else {
      setPage(false);
      updateInfo();
    }
  }

  if (apiState === 'ERROR') return message(apiError)
  else if (apiState !== 'READY') return loader('Connecting to Substrate')

  if (keyringState !== 'READY') {
    return loader(
      "Loading accounts (please review any extension's authorization)"
    )
  }

  const contextRef = createRef()

if (!page) {


  return (
    <div ref={contextRef}>
      <button onClick={handlePage}>Next Page</button>
      <Sticky context={contextRef}>
        <AccountSelector />
      </Sticky>
      <Container>
        <Grid stackable columns="equal">
          <Grid.Row stretched>
            <NodeInfo />
            <Metadata />
            <BlockNumber />
            <BlockNumber finalized />
          </Grid.Row>
          <Grid.Row stretched>
            <Balances />
          </Grid.Row>
          <Grid.Row>
            <Transfer />
            <Upgrade />
          </Grid.Row>
          <Grid.Row>
            <Interactor />
            <Events />
          </Grid.Row>
          <Grid.Row>
            <TemplateModule />
          </Grid.Row>
        </Grid>
      </Container>
      <DeveloperConsole />
    </div>
  )
} else {
  return (
    <div className='vrmeta'>
    <h1 className='title'>VRMETA Chain</h1>
    <Container className='container'>
      <div className='info'>
        <div className='row'>
        <div className='column stats'>
          <h5>Logged in as: {mainAccount}</h5>
         <p>VRMETA balance: {balance}</p>
         <p>VRMETA Timestamp: {time}</p>
         <p>Reward Rate:  {hourlyReward / 1000000000} VRMETA per hour</p>
         <p>Connected?:  {connected}</p>
         <p>Time Played: {myTimePlayed} minutes</p>
         <p>Mining Reward: {rewardToExpect} VRMETA</p>
          </div>
      
      <div className='column'>
      <BlockNumber/>
        </div>
        </div>
      
      <button className='vrbutton' onClick={connect} >Connect to the Metaverse</button>
      <button className='vrbutton' onClick={disconnect} >Disconnect from the Metaverse</button>
      </div>
      <Generate />
    <Grid stackable columns="equal">
      <Grid.Row stretched>    
        
      </Grid.Row>
      <Grid.Row>
        <Transfer />
      </Grid.Row>
      <Grid.Row>
        <Events />
      </Grid.Row>
    </Grid>
  </Container>
  </div>
  )
}
};

export default function App() {
  return (
    <SubstrateContextProvider>
      <Main />
    </SubstrateContextProvider>
  )
}
