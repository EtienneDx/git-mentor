import useAuthentication from "../hooks/useAuthentication";

const AuthenticationForm: React.FC = () => {
    const { token, credentials, handleFormSubmit, handleInputChange, error } =
        useAuthentication();

    return (
        <>
            {token ? (
                <div>
                    <p>Authenticated!</p>
                    <p>Token: {token}</p>
                </div>
            ) : (
                <form
                    onSubmit={handleFormSubmit}
                    className="flex flex-col gap-4"
                >
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
                    {error && <p style={{ color: "red" }}>{error}</p>}
                </form>
            )}
        </>
    );
};

export default AuthenticationForm;
