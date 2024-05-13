import React from "react";
import { AuthenticationProvider } from "./authentication";
import { ApiProvider } from "./api";
import { GlobalConfigProvider } from "./globalConfig";

const CombinedContextProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  return (
    <GlobalConfigProvider>
      <AuthenticationProvider>
        <ApiProvider>{children}</ApiProvider>
      </AuthenticationProvider>
    </GlobalConfigProvider>
  );
};

export default CombinedContextProvider;
