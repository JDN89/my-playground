package com.jdncodes.restapiplayground.repository;

import com.jdncodes.restapiplayground.entity.Department;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface DepartmentRepository extends JpaRepository<Department , Long> {
    public Department findByDepartmentName(String departmentName);

}
