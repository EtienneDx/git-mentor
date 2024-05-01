import React, { useEffect, useState } from "react";
import { main_service } from "../gmt-api";
import { useAuthentication } from "./authentication";

const ApiContext = React.createContext<main_service>(
  null as unknown as main_service
);

export default ApiContext;

export const useApi = () => React.useContext(ApiContext).default;

export const ApiProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const { token } = useAuthentication();
  const [api, setApi] = useState<main_service>(
    new main_service({
      TOKEN: token,
      BASE: process.env.API_BASE,
    })
  );

  useEffect(() => {
    setApi(
      new main_service({
        TOKEN: token,
        BASE: process.env.API_BASE,
      })
    );
  }, [token]);

  return <ApiContext.Provider value={api}>{children}</ApiContext.Provider>;
};
