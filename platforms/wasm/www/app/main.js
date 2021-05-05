import init, { getCodecs } from './vpiper.js';

      

(async () => {
  await init();
  console.log('result=', getCodecs());
})();



