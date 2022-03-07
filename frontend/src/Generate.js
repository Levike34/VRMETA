import { mnemonicGenerate } from '@polkadot/util-crypto';
import Swal from 'sweetalert2'

function generateMnemonic() {
    const mnemonic = mnemonicGenerate(12);
    Swal.fire({
        title: 'Seed Phrase:',
        text: mnemonic,
        color: 'black',
      })
    
}

export default function Generate() {
    return(
        <div>
            <button onClick={generateMnemonic}>New Account</button>
        </div>
    )
}