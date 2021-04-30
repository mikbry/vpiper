async function fetchAndInstantiate(url, importObject) {
  const response = await fetch(url);
  const bytes = await response.arrayBuffer();
  return WebAssembly.instantiate(bytes, importObject);
 }

(async () => {
  const { instance } = await fetchAndInstantiate('app/videopipe.wasm');
  console.log('result=', instance.exports.add(17, 25));
})();



