require("@nomicfoundation/hardhat-toolbox");
require('dotenv').config({ path: '../.env' })

const NETWORK_URL = process.env.NETWORK_URL;
const PRIVATE_KEY = process.env.PRIVATE_KEY;

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: "0.8.28",
  defaultNetwork: "localhost",
  networks: {
    localhost: {
      url: "http://127.0.0.1:8545",
    },
    sepolia:{
      url: NETWORK_URL,
      accounts: [PRIVATE_KEY]
    }
  },
  paths: {
    artifacts: "../artifacts"
  }
};
