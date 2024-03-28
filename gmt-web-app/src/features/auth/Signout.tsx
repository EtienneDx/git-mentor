import { useNavigate } from "react-router-dom";
import { useAuthentication } from "../../context/authentication";
import { useEffect } from "react";

const Signout: React.FC = () => {
  const { setToken } = useAuthentication();
  const navigate = useNavigate();
  useEffect(() => {
    setToken(undefined);
    navigate("/login");
  }, [setToken, navigate]);
  return <></>;
};

export default Signout;
