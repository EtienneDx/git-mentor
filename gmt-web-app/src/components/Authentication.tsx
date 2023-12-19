import React, { useState } from 'react';
import axios from 'axios';

interface Credentials {
  email: string;
  password: string;
}

interface AuthResponse {
  token: string;
}

const Authentication: React.FC = () => {
  const [credentials, setCredentials] = useState<Credentials>({
    email: '',
    password: '',
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
      const response = await axios.post<AuthResponse>('/api/auth/login', credentials);
      const authToken = response.data.token;
      setToken(authToken);
      setError(null);
    } catch (error) {
      setToken(null);
      setError('Invalid email or password');
    }
  };

  return (
    <div className="flex flex-col flex-wrap content-center justify-center h-screen w-full gap-2">
      <h1 className="font-bold text-2xl" >Email-Password Authentication</h1>
      {token ? (
        <div>
          <p>Authenticated!</p>
          <p>Token: {token}</p>
        </div>
      ) : (
        <form onSubmit={handleFormSubmit} className="flex flex-col gap-4">
          <label>
            Email:
            <input
              type="email"
              name="email"
              value={credentials.email}
              onChange={handleInputChange}
              required
            />
          </label>
          <label>
            Password:
            <input
              type="password"
              name="password"
              value={credentials.password}
              onChange={handleInputChange}
              required
            />
          </label>
          <button type="submit">Login</button>
          {error && <p style={{ color: 'red' }}>{error}</p>}
        </form>
      )}
    </div>
  );
};

export default Authentication;