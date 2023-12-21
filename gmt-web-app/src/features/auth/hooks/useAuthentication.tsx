import React, { useState } from "react";
import { login } from "../helpers/services";
import { useNavigate } from "react-router-dom";

interface Credentials {
  email: string;
  password: string;
}

const useAuthentication = () => {
  const navigate = useNavigate();
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
      const data = await login(credentials);
      setToken(data.token);
      setError(null);
      navigate('/groups', { replace: true });
    } catch (error) {
      setToken(null);
      setError("Invalid email or password");
    }
  };

  return { credentials, token, error, handleInputChange, handleFormSubmit };
};

export default useAuthentication;
