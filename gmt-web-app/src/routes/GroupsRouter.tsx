import { Route, Routes } from "react-router-dom";
import GroupsList from "../features/data-viz/pages/GroupsList";
import GroupAssignements from "../features/data-viz/pages/GroupAssignements";
import GroupStudents from "../features/data-viz/pages/GroupStudents";

const GroupsRouter = () => {
  return (
    <Routes>
      <Route index Component={GroupsList} />
      <Route path=":groupId" Component={GroupAssignements} />
      <Route path=":groupId/students" Component={GroupStudents} />
    </Routes>
  );
};

export default GroupsRouter;
