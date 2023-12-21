import { useState, useEffect } from "react";
import GroupsAPI from "../services/groupsAPI";
import { Group } from "../../../helpers/types";
import { ColumnHeader } from "../components/ColumnHeader";
import { ColumnContent } from "../components/ColumnContent";
import { useNavigate } from "react-router-dom";

const Groups = () => {
  const [groups, setGroups] = useState<Group[]>([]);
  const navigate = useNavigate();

  const onRowClick = (id: string) => {
    navigate("/groups/" + id);
  };

  useEffect(() => {
    const loadGroups = async () => {
      const newGroups: Group[] = await GroupsAPI.fetch();
      setGroups(newGroups);
    };
    loadGroups();
  }, []);

  return (
    <div>
      <h1>My Groups</h1>
      {groups.length === 0 ? (
        <h1>Loading Groups...</h1>
      ) : (
        <div className="flex flex-col justify-stretch">
          <table>
            <tr>
              <ColumnHeader title="Name" />
              <ColumnHeader title="Creation Date" />
            </tr>
            {groups.map((group) => (
              <tr
                key={group.id}
                onClick={() => onRowClick(group.id)}
                className="cursor-pointer"
              >
                <ColumnContent content={group.name} />
                <ColumnContent content={group.creationDate.toDateString()} />
              </tr>
            ))}
          </table>
        </div>
      )}
    </div>
  );
};

export default Groups;
