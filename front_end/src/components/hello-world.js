import React, {useEffect, useState} from 'react';
import axios from 'axios';
import config from "./config";

const HelloWorld = () => {
    const [data, setData] = useState(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);

    useEffect(() => {
        // Function to fetch data from the backend
        const fetchData = async () => {
            try {
                const response = await axios.get(`${config.backendUrl}/hello-world`);
                setData(response.data);
            } catch (err) {
                setError(err.message);
            } finally {
                setLoading(false);
            }
        };

        fetchData().then();
    }, []);

    if (loading) {
        return <div>Loading...</div>;
    }

    if (error) {
        return <div>Error: {error}</div>;
    }

    return (
        <div>
            <h1>Data from Backend</h1>
            <pre>{JSON.stringify(data, null, 2)}</pre>
        </div>
    );
};

export default HelloWorld;
