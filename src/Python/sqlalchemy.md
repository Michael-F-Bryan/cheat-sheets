SqlAlchemy Basics
=================

Engines and Sessions
--------------------

The docs for [engines][engines] and [sessions][sessions].

You can create a basic sqlite engine.

````python
    from sqlalchemy import create_engine
    engine = create_engine('sqlite:///:memory:')
````

Or for something more advanced...

Given:
* user: scott
* password: tiger
* host: localhost
* database: test

The equivalent URI would be:

````python
    engine = create_engine("postgresql://scott:tiger@localhost/test")
    engine = create_engine("mysql://scott:tiger@hostname/test",
                                encoding='latin1', echo=True)
````

To interact with the engine you need to use a session. You can create a session
using a sessionmaker.

````python
    from sqlalchemy.orm import sessionmaker
    Session = sessionmaker(bind=engine)

    some_session = Session()
````


Models
------

[The Docs][models]

An example of a basic model...

```python
    from sqlalchemy.ext.declarative import declarative_base
    from sqlalchemy import Column, Integer, String

    Base = declarative_base()

    class User(Base):
        __tablename__ = 'users'

        id = Column(Integer, primary_key=True)
        name = Column(String)
        fullname = Column(String)
        password = Column(String)

        def __repr__(self):
        return "<User(name='%s', fullname='%s', password='%s')>" % (
                                self.name, self.fullname, self.password)
```                             


It's pretty easy to create the tables that correspond to your models.

```python
    Base.metadata.create_all(engine)
```

Dropping all tables and data is just as simple.

```python
    Base.metadata.drop_all(engine)
```


Creating Data
-------------

[The Docs][creating]
Once you have a session and have defined some models, you can create instances
of that model.

```python
    ed_user = User(name='ed', fullname='Ed Jones', password='edspassword')
```

You can then stage the instance to your session (similar to how you stage
changes before committing them with `git`).

```python
    session.add(ed_user)
```

You can then commit everything in one hit.

```python
    session.commit()
```

Or roll back, if you don't want to save your changes.

```python
    session.rollback()
```


Making Queries
--------------

[The Docs][querying]


Many-to-Many Relationships
--------------------------

This can be a tricky topic, so make sure to read [the docs][many2many]!

Many to Many adds an association table between two classes. The association
table is indicated by the secondary argument to relationship(). Usually, the
Table uses the MetaData object associated with the declarative base class, so
that the ForeignKey directives can locate the remote tables with which to link.

```python
    from sqlalchemy import Table, Column, Integer, ForeignKey
    from sqlalchemy.orm import relationship
    from sqlalchemy.ext.declarative import declarative_base

    association_table = Table('association', Base.metadata,
        Column('left_id', Integer, ForeignKey('left.id')),
        Column('right_id', Integer, ForeignKey('right.id')))
```

Now to link the two classes, you need to declare a `relationship` attribute.
The `secondary` argument specifies the association table used. The
`back_populates` argument tells SqlAlchemy which column to link with when it
joins the two tables. It allows you to access the linked records as a list
with something like `Parent.children`.

```python
    class Parent(Base):
        __tablename__ = 'left'
        id = Column(Integer, primary_key=True)
        children = relationship(
            "Child",
            secondary=association_table,
            back_populates="parents")

    class Child(Base):
        __tablename__ = 'right'
        id = Column(Integer, primary_key=True)
        parents = relationship(
            "Parent",
            secondary=association_table,
            back_populates="children")
```

Alternatively, you can use the `backref` argument when specifying the
`relationship` for one class, this creates a virtual column in the
corresponding class that links back to the first one.

```python
    class Parent(Base):
        __tablename__ = 'left'
        id = Column(Integer, primary_key=True)
        children = relationship("Child",
                        secondary=association_table,
                        backref="parents")

    class Child(Base):
        __tablename__ = 'right'
        id = Column(Integer, primary_key=True)
```



[engines]: http://docs.sqlalchemy.org/en/latest/core/engines.html#database-urls
[sessions]: http://docs.sqlalchemy.org/en/latest/orm/session_basics.html#session-basics
[models]: http://docs.sqlalchemy.org/en/latest/orm/tutorial.html#declare-a-mapping
[creating]: http://docs.sqlalchemy.org/en/latest/orm/tutorial.html#adding-and-updating-objects
[querying]: http://docs.sqlalchemy.org/en/latest/orm/query.html#query-api
[many2many]: http://docs.sqlalchemy.org/en/latest/orm/basic_relationships.html#many-to-many
