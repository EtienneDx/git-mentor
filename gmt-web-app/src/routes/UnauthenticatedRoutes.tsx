import { Navigate, Route, Routes } from "react-router-dom";
import Login from "../features/auth/Login";
import Signup from "../features/auth/Signup";

const UnauthenticatedRoutes: React.FC = () => {
  return (
    <Routes>
      <Route path="/login" element={<Login />} />
      <Route path="/signup" element={<Signup />} />
      <Route path="*" element={<Navigate to="/login" />} />
    </Routes>
  );
};

export default UnauthenticatedRoutes;
