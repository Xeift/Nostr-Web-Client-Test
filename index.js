import init from './pkg/web_application.js';
init();
import * as wasm from './pkg/web_application.js';

const helloBtn = document.getElementById("helloBtn");
helloBtn.addEventListener("click", function() {
    console.log(wasm.greet())}
);

const sendMessageBtn = document.getElementById("sendMessageBtn");
sendMessageBtn.addEventListener("click", function() {
    const PRIVATEKEY = document.getElementById('privatekeyInput').value;
    console.log(PRIVATEKEY);
    console.log(wasm.send_message(PRIVATEKEY))}
    // nsec1tacg5mhajrp3usad50rne7r4st8j4chsdydjmkjhmeh83erc34hsxxfffc
);