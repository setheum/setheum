// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

import { ethers } from "hardhat";
import type { HardhatRuntimeEnvironment } from "hardhat/types";
import type { DeployFunction } from "hardhat-deploy/dist/types";

import { generateAccessControlFuncSignatures, verifyContract } from "../utils";

const deployFunc: DeployFunction = async function ({
  getNamedAccounts,
  deployments,
}: HardhatRuntimeEnvironment): Promise<void> {
  const { deploy } = deployments;
  const { deployer } = await getNamedAccounts();

  const domainID = process.env.BRIDGE_DOMAIN_ID;
  const reannounceAdminAddress =
    process.env.FUNCTIONS_ACCESS_ADMIN_ADDRESS ?? deployer;
  const spectreDomainIDs = process.env.SPECTRE_DOMAIN_IDS ?? 0;
  const spectreAddresses = process.env.SPECTRE_ADDRESSES ?? ethers.ZeroAddress;

  try {
    if (!domainID) throw Error("BRIDGE_DOMAIN_ID must be defined");
    if (!spectreDomainIDs) throw Error("SPECTRE_DOMAIN_IDS must be defined");
    if (!spectreAddresses) throw Error("SPECTRE_ADDRESSES must be defined");

    const contractsToGenerateSignatures = ["Bridge", "Router", "Executor"];
    const accessControlFuncSignatures = generateAccessControlFuncSignatures(
      contractsToGenerateSignatures,
    ).map((e: { function: string; hash: string }) => e.hash);

    console.log(
      `Initiated deploying contract to: ${deployments.getNetworkName()}`,
    );

    const accessControlArgs = [
      accessControlFuncSignatures,
      Array(accessControlFuncSignatures.length).fill(reannounceAdminAddress),
    ];
    const accessControlInstance = await deploy("AccessControlSegregator", {
      from: deployer,
      args: accessControlArgs,
      log: true,
    });
    await verifyContract(accessControlInstance, accessControlArgs);
    console.log(
      `Access control segregator contract successfully deployed to: ${accessControlInstance.address}`,
    );
    console.log(
      `Granted all admin functions rights to: ${reannounceAdminAddress}`,
    );

    const specterProxyArgs = [spectreDomainIDs, spectreAddresses];
    const spectreProxyInstance = await deploy("SpectreProxy", {
      from: deployer,
      args: specterProxyArgs,
      log: true,
    });
    await verifyContract(spectreProxyInstance, specterProxyArgs);
    console.log(
      `Spectre proxy contract successfully deployed to: ${spectreProxyInstance.address}`,
    );

    const bridgeArgs = [domainID, accessControlInstance.address];
    const bridgeInstance = await deploy("Bridge", {
      from: deployer,
      args: bridgeArgs,
      log: true,
    });
    await verifyContract(bridgeInstance, bridgeArgs);
    console.log(
      `Bridge contract successfully deployed to: ${bridgeInstance.address}`,
    );

    const routerArgs = [bridgeInstance.address, accessControlInstance.address];
    const routerInstance = await deploy("Router", {
      from: deployer,
      args: routerArgs,
      log: true,
    });
    await verifyContract(routerInstance, routerArgs);
    console.log(
      `Router contract successfully deployed to: ${routerInstance.address}`,
    );

    const executorArgs = [
      bridgeInstance.address,
      accessControlInstance.address,
    ];
    const executorInstance = await deploy("Executor", {
      from: deployer,
      args: executorArgs,
      log: true,
    });
    await verifyContract(executorInstance, executorArgs);
    console.log(
      `Executor contract successfully deployed to: ${executorInstance.address}`,
    );
  } catch (error) {
    console.error(
      `Deploying core contracts failed because of:${(error as Error).stack}`,
    );
  }
};

export default deployFunc;
