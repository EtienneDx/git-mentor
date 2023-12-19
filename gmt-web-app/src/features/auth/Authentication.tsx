import useAuthentication from "./hooks/useAuthentication";

const Authentication: React.FC = () => {
  const {token, credentials, handleFormSubmit, handleInputChange, error} = useAuthentication();

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