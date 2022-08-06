package com.jdncodes.restapiplayground.entity;

import lombok.*;
import org.hibernate.Hibernate;

import javax.persistence.*;
import java.util.Objects;

@Entity
@Data
@NoArgsConstructor
@AllArgsConstructor
@Builder
//@EqualsAndHashCode

public class Department {
@Id
@GeneratedValue(strategy = GenerationType.AUTO)
    private Long id;
@Column(nullable = false, unique = true)
    private String departmentName;
    private Short numberOfEmployees;
    private String Address;

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || Hibernate.getClass(this) != Hibernate.getClass(o)) return false;
        Department that = (Department) o;
        return id != null && Objects.equals(id, that.id);
    }

    @Override
    public int hashCode() {
        return getClass().hashCode();
    }
}
