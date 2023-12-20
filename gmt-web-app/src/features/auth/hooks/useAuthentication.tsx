import React, { useState } from "react";
import axios from "axios";

interface Credentials {
  email: string;
  password: string;
}

interface AuthResponse {
  token: string;
}

const useAuthentication = () => {
  const [credentials, setCredentials] = useState<Credentials>({
    email: "",
    password: "",
  });
  const [token, setToken] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setCredentials((prevCredentials) => ({
      ...prevCredentials,
      [name]: value,
    }));
  };

  const handleFormSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    try {
      const response = await axios.post<AuthResponse>(
        "/api/auth/login",
        credentials
      );
      const authToken = response.data.token;
      setToken(authToken);
      setError(null);
    } catch (error) {
      setToken(null);
      setError("Invalid email or password");
    }
  };

  return { credentials, token, error, handleInputChange, handleFormSubmit };
};

export default useAuthentication;
