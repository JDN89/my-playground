package com.jdncodes.restapiplayground.entity;

import lombok.*;

import javax.persistence.*;

@Entity
@Data
@NoArgsConstructor
@AllArgsConstructor
@Builder

public class Department {
@Id
@GeneratedValue(strategy = GenerationType.AUTO)
    private Long id;
@Column(nullable = false, unique = true)
    private String departmentName;
    private Short numberOfEmployees;
    private String Address;
}
