import { useState, useEffect } from "react";
import GroupsAPI from "../services/groupsAPI";
import { Group } from "../../../helpers/types";

const Groups = () => {
  const [groups, setGroups] = useState<Group[]>([]);

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
        <div>
          {groups.map((group) => (
            <div key={group.id}>{group.name}</div>
          ))}
        </div>
      )}
    </div>
  );
};

export default Groups;
