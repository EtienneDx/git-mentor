import React from "react";
import { useAuthentication } from "../../../context/authentication";
import { useApi } from "../../../context/api";
import { useNavigate } from "react-router-dom";
import { ApiError } from "../../../gmt-api";

const LoginForm: React.FC = () => {
  const { setToken, stayLoggedIn, setStayLoggedIn } = useAuthentication();
  const [credentials, setCredentials] = React.useState({
    email: "",
    password: "",
  });
  const [error, setError] = React.useState<string>();
  const api = useApi();
  const navigate = useNavigate();

  const handleFormSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    try {
      const response = await api.postLogin(credentials);
      setToken(response.token);
      navigate("/");
    } catch (error) {
      if (error instanceof ApiError) {
        setError("Invalid email or password");
      } else {
        setError("An unexpected error occurred");
      }
    }
  };

  return (
    <form onSubmit={handleFormSubmit} className="flex flex-col gap-4">
      <label>
        Email:
        <input
          type="email"
          name="email"
          value={credentials.email}
          onChange={(e) =>
            setCredentials({ ...credentials, email: e.target.value })
          }
          required
        />
      </label>
      <label>
        Password:
        <input
          type="password"
          name="password"
          value={credentials.password}
          onChange={(e) =>
            setCredentials({ ...credentials, password: e.target.value })
          }
          required
        />
      </label>
      <label>
        <input
          type="checkbox"
          name="stayLoggedIn"
          checked={stayLoggedIn}
          onChange={(e) => setStayLoggedIn(e.target.checked)}
        />
        Stay logged in
      </label>
      <button type="submit">Login</button>
      {error && <p style={{ color: "red" }}>{error}</p>}
    </form>
  );
};

export default LoginForm;
