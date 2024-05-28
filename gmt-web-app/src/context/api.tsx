import React, { useEffect, useState } from "react";
import { main_service } from "../gmt-api";
import { useAuthentication } from "./authentication";
import { useGlobalConfig } from "./globalConfig";

const ApiContext = React.createContext<main_service>(
  null as unknown as main_service
);

export default ApiContext;

export const useApi = () => React.useContext(ApiContext).default;

export const ApiProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const { token } = useAuthentication();
  const { apiBase } = useGlobalConfig();

  const [api, setApi] = useState<main_service>(
    new main_service({
      TOKEN: token,
      BASE: apiBase,
    })
  );

  useEffect(() => {
    setApi(
      new main_service({
        TOKEN: token,
        BASE: apiBase,
      })
    );
  }, [token, apiBase]);

  return <ApiContext.Provider value={api}>{children}</ApiContext.Provider>;
};
