// This setup uses Hardhat Ignition to manage smart contract deployments.
// Learn more about it at https://hardhat.org/ignition

const { buildModule } = require("@nomicfoundation/hardhat-ignition/modules");
require('dotenv').config({ path: '../../../.env' })

const INITIAL_OWNER = process.env.INITIAL_OWNER;

module.exports = buildModule("GenesisTokenModule", (m) => {
  const initialOwner = m.getParameter("initialOwner", INITIAL_OWNER);

  const genesisToken = m.contract("GenesisToken", [initialOwner]);

  return { genesisToken };
});
