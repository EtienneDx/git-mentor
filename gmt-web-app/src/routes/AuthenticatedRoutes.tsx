import { Route, Routes } from "react-router-dom";
import { useTokenData } from "../context/authentication";
import Signout from "../features/auth/Signout";
import UserRoutes from "./UserRoutes";
import StudentsRouter from "./StudentsRouter";
import PageLayout from "../components/organisms/PageLayout";

const AuthenticatedRoutes: React.FC = () => {
  const data = useTokenData();
  return (
    <PageLayout>
      <Routes>
        <Route index element={<UserRoutes />} />
        <Route path={`/${data.user_id}/*`} element={<UserRoutes />} />
        <Route path="/students/*" element={<StudentsRouter />} />
        <Route path="/signout" element={<Signout />} />
      </Routes>
    </PageLayout>
  );
};

export default AuthenticatedRoutes;
