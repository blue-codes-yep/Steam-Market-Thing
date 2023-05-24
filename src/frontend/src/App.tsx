import React, { useEffect, useState } from 'react';

interface Item {
  amount_of_items: string;
  starting_at: string;
  lowest_price: string;
  item: string;
  game: string;
  image_url: string,
}

const ItemList: React.FC = () => {
  const [items, setItems] = useState<Item[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    setIsLoading(true);
    fetch('http://localhost:8000/items')
      .then(response => {
        if (!response.ok) {
          throw new Error('Network response was not ok');
        }
        return response.json();
      })
      .then(data => {
        setItems(data);
        setIsLoading(false);
      })
      .catch(error => {
        setError(error);
        setIsLoading(false);
      });
  }, []);

  if (isLoading) {
    return <div>Loading...</div>;
  }

  if (error) {
    return <div>Error: {error.message}</div>;
  }
  return (
    <div style={{ overflowY: 'scroll', maxHeight: '500px' }}>
        {items.map((item, index) => (
            <div key={index} style={{ display: 'flex', alignItems: 'center', marginBottom: '10px' }}>
                <img src={item.image_url} alt={item.item} style={{ marginRight: '10px' }} />
                <div>
                    <h2>{item.item}</h2>
                    <p>Amount of items: {item.amount_of_items}</p>
                    <p>Starting at: {item.starting_at}</p>
                    <p>Lowest price: {item.lowest_price}</p>
                    <p>Game: {item.game}</p>
                </div>
            </div>
        ))}
    </div>
  );
};

export default ItemList;