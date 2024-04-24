import { Route, Routes } from "react-router-dom";
import { useTokenData } from "../context/authentication";
import Signout from "../features/auth/Signout";
import GroupsRouter from "./GroupsRouter";
import RepositoriesRouter from "./RepositoriesRouter";
import StudentsRouter from "./StudentsRouter";

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
      <Route path="/groups/*" element={<GroupsRouter />} />
      <Route path="/students/*" element={<StudentsRouter />} />
      <Route path="/repositories/*" element={<RepositoriesRouter />} />
      <Route path="/about" element={<p>About</p>} />
      <Route path="/signout" element={<Signout />} />
    </Routes>
  );
};

export default AuthenticatedRoutes;
