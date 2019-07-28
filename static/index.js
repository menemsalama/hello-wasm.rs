const encoder = new TextEncoder();

function bufferToHex(buffer) {
  return Array
  .from(new Uint8Array (buffer))
  .map(b => b.toString (16).padStart(2, "0"))
  .join("");
}

async function hash(str) {
  return window.crypto.subtle.digest(
      {
          name: "SHA-1",
      },
      encoder.encode(str),
  )
  .then(function(hash){
      return bufferToHex(hash);
  })
}

async function mine(n) {
  const start = performance.now();

  let nonce = 0;
  const times = n > 4 ? 4 : n < 1 ? 1 : n;
  const zeroes = "0".repeat(times);

  while (true) {
    let msg = `The winner is Wasm, Nonce is ${nonce}`;
    let hashed_msg = await hash(msg);

    nonce++;
    if (hashed_msg.startsWith(zeroes)) {
      const duration = (performance.now() - start);
      console.log(`Javascript duration in milliseconds ${duration}`);
      return hashed_msg;
    }
  }
}

async function test(wasm, n = 2) {
  const output = {
    js: document.getElementById( "output.js" ),
    rs: document.getElementById( "output.rs" ),
  };

  output.js.innerText = "";
  output.rs.innerText = "";

  let results = {};
  let timing = {};

  const rs = performance.now();
  results.rs = wasm.mine(n);
  timing.rs = "Call mine.rs took " + (performance.now() - rs) + " milliseconds.";

  const js = performance.now();
  results.js = await mine(n);
  timing.js = "Call mine.js took " + (performance.now() - js) + " milliseconds.";

  output.js.innerText = `
    Digest = ${results.js}
    Time in ms = ${timing.js}
  `;
  output.rs.innerText = `
    Digest = ${results.rs}
    Time in ms = ${timing.rs}
  `;

}

Rust.hello_wasm.then( async function( wasm ) {
  console.log("wasm", wasm);
  test(wasm)

    var input = document.getElementById( "input" );
    input.addEventListener( "change", async function( event ) {
        input.disabled = true;
        let n = parseInt(input.value);
        n = isNaN(n) ? 1 : n;
        console.log("input.value ", n);
        await test(wasm, n);
        input.disabled = false;
    });
});
