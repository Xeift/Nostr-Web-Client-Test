import init from '../pkg/web_application.js';
init();
import * as wasm from '../pkg/web_application.js';


const helloBtn = document.getElementById('helloBtn');
helloBtn.addEventListener('click', function() {
    console.log(wasm.greet())
});


const sendMessageBtn = document.getElementById('sendMessageBtn');
sendMessageBtn.addEventListener('click', function() {
    const PRIVATEKEY = localStorage.getItem('NOSTR_PRIVATE_KEY');
    console.log('------------------------');
    console.log(PRIVATEKEY);
    console.log(wasm.send_message(PRIVATEKEY))
    console.log('------------------------\n');
});

// nsec1tacg5mhajrp3usad50rne7r4st8j4chsdydjmkjhmeh83erc34hsxxfffc
const storePrivatekeyBtn = document.getElementById('storePrivatekeyBtn');
storePrivatekeyBtn.addEventListener('click', function() {
    const PRIVATEKEY = document.getElementById('privatekeyInput').value;
    localStorage.setItem('NOSTR_PRIVATE_KEY', PRIVATEKEY)
})