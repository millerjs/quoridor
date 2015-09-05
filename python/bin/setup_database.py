from quoridor.models import Base
from sqlalchemy import create_engine
import argparse


if __name__ == '__main__':

    parser = argparse.ArgumentParser()
    parser.add_argument('-H', "--host", type=str, action="store",
                        default='localhost', help="psql-server host")
    parser.add_argument('-u', "--user", type=str, action="store",
                        default='test', help="psql test user")
    parser.add_argument('-d', "--database", type=str, action="store",
                        default='automated_test', help="psql test database")

    args = parser.parse_args()
    engine = create_engine("postgres://{user}@{host}/{db}".format(
        user=args.user, host=args.host, db=args.database))
    Base.metadata.create_all(engine)
