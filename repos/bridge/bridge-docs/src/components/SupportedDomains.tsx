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
