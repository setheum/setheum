import React, { useEffect, useState } from 'react';
import { Config, Environment } from '@buildwithsygma/sygma-sdk-core';
import { capName } from '../utils';

type SupportedDomainsProps = {
  environment: Environment;
};

const SupportedDomains: React.FC<SupportedDomainsProps> = ({ environment }) => {
    const [config, setConfig] = useState<Config | null>(null);

    useEffect(() => {
      const initializeConfig = async () => {
        try {
          const config = new Config();
          await config.init(1, environment);
          setConfig(config);
        } catch (error) {
          console.error('Error initializing config: ', error);
        }
      };

      initializeConfig();
  }, []);

  if (!config || !config.environment) {
    return <div>Loading domains...</div>;
  }

  return (
    <>
    <table>
      <thead>
        <tr>
          <th>Network Name</th>
          <th>Type</th>
          <th>Sygma Domain ID</th>
          <th>Chain ID</th>
        </tr>
      </thead>
      <tbody>
        {config.environment.domains.map((domain, index) => (
          <tr key={index}>
            <td>{capName(domain.name)}</td>
            <td>{domain.type.toUpperCase()}</td>
            <td>{domain.id}</td>
            <td>{domain.chainId}</td>
          </tr>
        ))}
      </tbody>
    </table>
    </>
  );
};

export default SupportedDomains;
