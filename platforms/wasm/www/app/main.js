async function fetchAndInstantiate(url, importObject) {
  const response = await fetch(url);
  const bytes = await response.arrayBuffer();
  return WebAssembly.instantiate(bytes, importObject);
 }

(async () => {
  const { instance } = await fetchAndInstantiate('app/vpiper.wasm');
  console.log('result=', instance.exports.getCodecs());
})();



