import { Navigate } from "react-router-dom";
import { useAuthentication } from "../../context/authentication";
import { useEffect } from "react";

const Signout: React.FC = () => {
  const { setToken } = useAuthentication();
  useEffect(() => {
    setToken(undefined);
  }, [setToken]);
  return <Navigate to="/login" replace />;
};

export default Signout;
