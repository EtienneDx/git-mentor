import React, { useState } from "react";
import { login } from "@/api/auth";
import { LoginRequest } from "@/proto";

const useAuthentication = () => {
  const [credentials, setCredentials] = useState<LoginRequest>({
    username: "",
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

    const response = await login(credentials);

    if (response.success) {
      setToken(response.data.token);
      setError(null);
    } else {
      setToken(null);
      setError(response.error.msg);
    }
  };

  return { credentials, token, error, handleInputChange, handleFormSubmit };
};

export default useAuthentication;
