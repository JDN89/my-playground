package com.example.demo.student;

import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

//Data access layer
//we're going to use inside app service
//@Repository for DI
@Repository
public interface StudentRepository extends JpaRepository<Student,Long> {
}
