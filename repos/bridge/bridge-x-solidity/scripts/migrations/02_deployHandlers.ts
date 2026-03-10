import type { HardhatRuntimeEnvironment } from "hardhat/types";
import type { DeployFunction } from "hardhat-deploy/dist/types";

import { verifyContract } from "../utils";

const deployFunc: DeployFunction = async function ({
  getNamedAccounts,
  deployments,
}: HardhatRuntimeEnvironment): Promise<void> {
  const { deploy } = deployments;
  const { deployer } = await getNamedAccounts();
  try {
    const bridgeInstance = await deployments.get("Bridge");
    const routerInstance = await deployments.get("Router");
    const executorInstance = await deployments.get("Executor");

    const erc20HandlerArgs = [
      bridgeInstance.address,
      routerInstance.address,
      executorInstance.address,
    ];
    const erc20HandlerInstance = await deploy("ERC20Handler", {
      from: deployer,
      args: erc20HandlerArgs,
      log: true,
    });
    await verifyContract(erc20HandlerInstance, erc20HandlerArgs);
    console.log(
      `ERC20 handler contract successfully deployed to: ${erc20HandlerInstance.address}`,
    );

    const permissionlessGenericHandlerArgs = [
      bridgeInstance.address,
      executorInstance.address,
    ];
    const permissionlessGenericHandlerInstance = await deploy(
      "PermissionlessGenericHandler",
      {
        from: deployer,
        args: permissionlessGenericHandlerArgs,
        log: true,
      },
    );
    await verifyContract(
      permissionlessGenericHandlerInstance,
      permissionlessGenericHandlerArgs,
    );
    console.log(
      `Permissionless generic handler contract successfully deployed to: ${permissionlessGenericHandlerInstance.address}`,
    );

    const feeHandlerRouterArgs = [routerInstance.address];
    const feeHandlerRouterInstance = await deploy("FeeHandlerRouter", {
      from: deployer,
      args: feeHandlerRouterArgs,
      log: true,
    });
    await verifyContract(feeHandlerRouterInstance, feeHandlerRouterArgs);
    console.log(
      `Fee handler router contract successfully deployed to: ${feeHandlerRouterInstance.address}`,
    );

    const basicFeeHandlerArgs = [
      bridgeInstance.address,
      feeHandlerRouterInstance.address,
      routerInstance.address,
    ];
    const basicFeeHandlerInstance = await deploy("BasicFeeHandler", {
      from: deployer,
      args: basicFeeHandlerArgs,
      log: true,
    });
    await verifyContract(basicFeeHandlerInstance, basicFeeHandlerArgs);
    console.log(
      `Basic fee handler contract successfully deployed to: ${basicFeeHandlerInstance.address}`,
    );

    const percentageFeeHandlerArgs = [
      bridgeInstance.address,
      feeHandlerRouterInstance.address,
      routerInstance.address,
    ];
    const percentageFeeHandlerInstance = await deploy(
      "PercentageERC20FeeHandlerEVM",
      {
        from: deployer,
        args: percentageFeeHandlerArgs,
        log: true,
      },
    );
    await verifyContract(
      percentageFeeHandlerInstance,
      percentageFeeHandlerArgs,
    );
    console.log(
      `Percentage fee handler contract successfully deployed to: ${percentageFeeHandlerInstance.address}`,
    );
  } catch (error) {
    console.error(
      `Deploying handler contracts failed because of:${(error as Error).stack}`,
    );
  }
};

export default deployFunc;
