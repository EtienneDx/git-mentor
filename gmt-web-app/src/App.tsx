import React from "react";
import ContextProvider from "./context";
import { useAuthentication } from "./context/authentication";
import { AuthenticatedRoutes, UnauthenticatedRoutes } from "./routes";
import { BrowserRouter } from "react-router-dom";

const AppWrapper: React.FC = () => {
  return (
    <div className="min-h-screen">
      <ContextProvider>
        <App />
      </ContextProvider>
    </div>
  );
};

const App: React.FC = () => {
  const { token } = useAuthentication();
  return (
    <BrowserRouter>
      {token ? <AuthenticatedRoutes /> : <UnauthenticatedRoutes />}
    </BrowserRouter>
  );
};

export default AppWrapper;
