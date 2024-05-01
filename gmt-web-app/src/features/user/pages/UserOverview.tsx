import React from 'react';
import { useTokenData } from '../../../context/authentication';


const UserOverview: React.FC = () => {
  const { username, email } = useTokenData();
  return (
    <div>
      <h1>User Overview</h1>
      <p>Welcome to the user overview page! This page displays information about the user, including their name, email, and profile picture.</p>
      <div className="user-info">
        <img src="/path/to/default-picture.png" alt="Default Profile Picture" />
        <div className="user-details">
          <h2>{username}</h2>
          <p>Email: {email}</p>
        </div>
      </div>
    </div>
  );
};

export default UserOverview;