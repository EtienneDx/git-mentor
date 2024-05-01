import { Route, Routes } from "react-router-dom";
import RepositoriesRouter from "./RepositoriesRouter";
import UserOverview from "../features/user/pages/UserOverview";
import GroupsRouter from "./GroupsRouter";

const UserRoutes: React.FC = () => {
  return (
    <Routes>
      <Route index element={<UserOverview />} />
      <Route path="/repositories/*" element={<RepositoriesRouter />} />
      <Route path="/groups/*" element={<GroupsRouter />} />
    </Routes>
  );
};

export default UserRoutes;
