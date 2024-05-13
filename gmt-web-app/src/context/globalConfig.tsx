import React, { useEffect, useState } from "react";

type GlobalConfigType = {
  apiBase?: string;
};

const defaultConfig: GlobalConfigType = {
  apiBase: undefined,
};

const GlobalConfig = React.createContext<GlobalConfigType>(defaultConfig);

export default GlobalConfig;

export const useGlobalConfig = () => React.useContext(GlobalConfig);

export const GlobalConfigProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [config, setConfig] = useState<GlobalConfigType>(defaultConfig);

  useEffect(() => {
    fetch("/config.json")
      .then((response) => response.json())
      .then((data) => {
        if (data.apiBase.startsWith("/")) {
          data.apiBase = window.location.origin + data.apiBase;
        }
        return data;
      })
      .then((data) => {
        setConfig(data);
      })
      .catch((_) => {
        setConfig(defaultConfig);
      });
  }, []);

  return <GlobalConfig.Provider value={config}>{children}</GlobalConfig.Provider>;
};
