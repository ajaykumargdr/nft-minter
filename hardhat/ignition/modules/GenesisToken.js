// This setup uses Hardhat Ignition to manage smart contract deployments.
// Learn more about it at https://hardhat.org/ignition

const { buildModule } = require("@nomicfoundation/hardhat-ignition/modules");

// TODO - take from hardhat.config.js or .env
const INITIAL_OWNER = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";

module.exports = buildModule("GenesisTokenModule", (m) => {
  const initialOwner = m.getParameter("initialOwner", INITIAL_OWNER);

  const genesisToken = m.contract("GenesisToken", [initialOwner]);

  return { genesisToken };
});
