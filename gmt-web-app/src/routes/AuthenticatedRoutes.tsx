import { Route, Routes } from "react-router-dom";
import { useTokenData } from "../context/authentication";
import Signout from "../features/auth/Signout";

const AuthenticatedRoutes: React.FC = () => {
  const data = useTokenData();
  return (
    <Routes>
      <Route
        path="/"
        element={
          <p>
            Welcome {data.username}. <a href="/signout">Sign out</a>
          </p>
        }
      />
      <Route path="/about" element={<p>About</p>} />
      <Route path="/signout" element={<Signout />} />
    </Routes>
  );
};

export default AuthenticatedRoutes;
