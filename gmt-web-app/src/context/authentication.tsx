import React from "react";
import { jwtDecode } from "jwt-decode";

const stayLoggedInStorage = localStorage.getItem("stayLoggedIn") === "true";
const tokenStorage = localStorage.getItem("token") ?? undefined;

const AuthenticationContext = React.createContext<{
  token?: string;
  setToken: (_?: string) => void;
  stayLoggedIn: boolean;
  setStayLoggedIn: (_: boolean) => void;
}>({
  token: tokenStorage,
  stayLoggedIn: stayLoggedInStorage,
  setToken: (_?: string) => {},
  setStayLoggedIn: (_: boolean) => {},
});

export default AuthenticationContext;

export const useAuthentication = () => React.useContext(AuthenticationContext);

export const AuthenticationProvider: React.FC<{
  children: React.ReactNode;
}> = ({ children }) => {
  const [token, setTokenInner] = React.useState<string | undefined>(
    tokenStorage
  );
  const [stayLoggedIn, setStayLoggedInInner] =
    React.useState<boolean>(stayLoggedInStorage);

  const setToken = (token?: string) => {
    if (stayLoggedIn) {
      if (token) {
        localStorage.setItem("token", token);
      } else {
        localStorage.removeItem("token");
      }
    }
    setTokenInner(token);
  };

  const setStayLoggedIn = (stayLoggedIn: boolean) => {
    localStorage.setItem("stayLoggedIn", stayLoggedIn ? "true" : "false");
    setStayLoggedInInner(stayLoggedIn);
  };

  return (
    <AuthenticationContext.Provider
      value={{ token, setToken, stayLoggedIn, setStayLoggedIn }}
    >
      {children}
    </AuthenticationContext.Provider>
  );
};

export const useTokenData = () => {
  const { token } = useAuthentication();

  // Decode the JWT token
  const tokenData = token ? jwtDecode(token) : undefined;

  // Return the decoded token data
  return tokenData as {
    username: string;
    email: string;
    pubkeys: string[];
    user_id: number;
  };
};
