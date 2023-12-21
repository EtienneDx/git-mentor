import axios from "axios";
import { Credentials, AuthResponse } from "./types";

export const login = async (credentials: Credentials) => {
  return {
    token: "token",
  };
  const response = await axios.post<AuthResponse>(
    "/api/auth/login",
    credentials
  );
  return response.data;
};
