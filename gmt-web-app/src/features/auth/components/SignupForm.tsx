import React from "react";
import { useAuthentication } from "../../../context/authentication";
import { useApi } from "../../../context/api";
import { useNavigate } from "react-router-dom";
import { ApiError } from "../../../context/types";

const SignupForm: React.FC = () => {
  const { setToken, stayLoggedIn, setStayLoggedIn } = useAuthentication();
  const [credentials, setCredentials] = React.useState({
    username: "",
    email: "",
    password: "",
    confirmPassword: "",
  });
  const [error, setError] = React.useState<string>();
  const api = useApi();
  const navigate = useNavigate();

  const handleFormSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (credentials.password !== credentials.confirmPassword) {
      setError("Passwords do not match");
      return;
    }

    try {
      const response = await api.postSignup({
        username: credentials.username,
        email: credentials.email,
        password: credentials.password,
      });
      setToken(response.token);
      navigate("/");
    } catch (error) {
      if (error instanceof ApiError) {
        setError(error.message);
      } else {
        setError("An unexpected error occurred");
      }
    }
  };

  return (
    <form onSubmit={handleFormSubmit} className="flex flex-col gap-4">
      <label>
        Username:
        <input
          type="text"
          name="username"
          value={credentials.username}
          onChange={(e) =>
            setCredentials({ ...credentials, username: e.target.value })
          }
          required
        />
      </label>
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
        Confirm Password:
        <input
          type="password"
          name="confirmPassword"
          value={credentials.confirmPassword}
          onChange={(e) =>
            setCredentials({ ...credentials, confirmPassword: e.target.value })
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
      <button type="submit">Signup</button>
      {error && <p style={{ color: "red" }}>{error}</p>}
    </form>
  );
};

export default SignupForm;
