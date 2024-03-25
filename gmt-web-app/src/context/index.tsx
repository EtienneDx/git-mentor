import React from "react";
import { AuthenticationProvider } from "./authentication";
import { ApiProvider } from "./api";

const CombinedContextProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  return (
    <AuthenticationProvider>
      <ApiProvider>{children}</ApiProvider>
    </AuthenticationProvider>
  );
};

export default CombinedContextProvider;
